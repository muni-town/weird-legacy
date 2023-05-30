<script lang="ts">
  import debounce from 'debounce'
  import { browser } from '$app/environment'
  import { goto } from '$app/navigation'
  import type { Octokit } from '@octokit/rest'
  import Icon from 'components/Icon.svelte'
  import Spinner from 'components/Spinner.svelte'
  import { GH_ACCESS_TOKEN_NAME } from 'config'
  import { getOctokit } from 'utils'
  import {
    GH_PAGES_DEPLOYMENT_TARGET_CONTENTS,
    GH_PAGES_DEPLOYMENT_TARGET_FILENAME,
  } from './consts'
  import githubWorkflow from './github-workflow.yaml?raw'
  import { invoke } from '@tauri-apps/api'

  let user:
    | Awaited<ReturnType<Octokit['users']['getAuthenticated']>>['data']
    | null = null

  if (browser) {
    const octokit = getOctokit()
    ;(async () => {
      const userResp = await octokit.users.getAuthenticated()
      user = userResp.data
    })()
  }

  let loading = false
  let repoStatus:
    | 'good_exists'
    | 'good_will_create'
    | 'bad'
    | 'unknown'
    | 'checking' = 'unknown'
  let errorMessage: string | null = null
  let repo = ''
  let domain = ''

  let statusMessage = ''
  $: {
    switch (repoStatus) {
      case 'bad':
        statusMessage = errorMessage || 'error'
        break
      case 'good_exists':
        statusMessage = 'Repo exists and will be re-deployed.'
        break
      case 'good_will_create':
        statusMessage = 'Repo does not exist and will be created.'
        break
      case 'checking':
      case 'unknown':
        statusMessage = ''
        break
    }
  }

  const logout = () => {
    if (browser) {
      localStorage.removeItem(GH_ACCESS_TOKEN_NAME)
      goto('/publish/github/login')
    }
  }

  const checkRepo = debounce(async () => {
    if (!browser) return
    errorMessage = null
    repoStatus = 'checking'

    const octokit = getOctokit()

    if (!repo) {
      repoStatus = 'unknown'
      return
    }

    try {
      // Verify this is a repo created by Weird, if it exists already
      const user = await octokit.users.getAuthenticated()
      let repoExists = false
      const error = await octokit.repos
        .get({ owner: user.data.login, repo })
        .then(async (_repoData) => {
          repoExists = true
          try {
            await octokit.repos.getContent({
              owner: user.data.login,
              repo,
              path: GH_PAGES_DEPLOYMENT_TARGET_FILENAME,
            })
          } catch (e) {
            return 'Repository exists, but does not appear to have been created by Weird.'
          }
        })
        .catch(() => undefined)

      if (error) {
        throw error
      }

      repoStatus = repoExists ? 'good_exists' : 'good_will_create'
    } catch (e) {
      errorMessage = e
      repoStatus = 'bad'
    }
  }, 300)

  // Check repo when it changes
  $: {
    ;(async () => {
      repoStatus = repo ? 'checking' : 'unknown'
      checkRepo()
    })()
  }

  async function createRepo({
    octokit,
    owner,
    repo,
  }: {
    octokit: Octokit
    owner: string
    repo: string
  }) {
    await octokit.request('POST /user/repos', {
      name: repo,
      description: 'Website deployed with Weird',
      has_issues: false,
      has_projects: false,
      has_wiki: false,
      has_discussions: false,
      has_downloads: false,
    })
    // Add the weird deployment file to mark the repo as being used for weird
    await octokit.repos.createOrUpdateFileContents({
      owner,
      repo,
      path: GH_PAGES_DEPLOYMENT_TARGET_FILENAME,
      message: 'Initial Commit',
      content: btoa(GH_PAGES_DEPLOYMENT_TARGET_CONTENTS),
    })
  }

  const deployImpl = async () => {
    const octokit = getOctokit()
    let zipFileBase64 = await invoke<string>('get_export_zip_base64')

    const owner = user.login
    const repoExists = repoStatus == 'good_exists'
    if (!repoExists) {
      await createRepo({ owner, repo, octokit })
    }

    // Get the repo data
    const repoData = await octokit.repos.get({ owner, repo })
    const defaultBranch = repoData.data.default_branch

    // Update the GitHub pages configuration for the repo
    if (!repoData.data.has_pages) {
      await octokit.repos.createPagesSite({
        owner,
        repo,
        build_type: 'workflow',
        source: {
          branch: defaultBranch,
          path: '/',
        },
      })
    }
    await octokit.repos.updateInformationAboutPagesSite({
      owner,
      repo,
      cname: domain || null,
      build_type: 'workflow',
      source: {
        branch: defaultBranch,
        path: '/',
      },
    })
    const ref = `heads/${defaultBranch}`

    const websiteZipBlob = await octokit.git.createBlob({
      owner,
      repo,
      content: zipFileBase64,
      encoding: 'base64',
    })
    const readme = await octokit.git.createBlob({
      owner,
      repo,
      content:
        '# Weird Website Deployment\n\nThis is a site that has been deployed with [Weird](https://github.com/commune-os/weird).',
    })
    const workflowsDir = await octokit.git.createTree({
      owner,
      repo,
      tree: [
        {
          path: 'deploy.yaml',
          mode: '100644',
          content: githubWorkflow.replace('$default-branch', defaultBranch),
        },
      ],
    })
    const githubDir = await octokit.git.createTree({
      owner,
      repo,
      tree: [
        {
          path: 'workflows',
          type: 'tree',
          mode: '040000',
          sha: workflowsDir.data.sha,
        },
      ],
    })
    const rootDir = await octokit.git.createTree({
      owner,
      repo,
      tree: [
        {
          path: '.github',
          type: 'tree',
          mode: '040000',
          sha: githubDir.data.sha,
        },
        {
          path: 'website.zip',
          mode: '100644',
          type: 'blob',
          sha: websiteZipBlob.data.sha,
        },
        {
          path: 'README.md',
          mode: '100644',
          type: 'blob',
          sha: readme.data.sha,
        },
        {
          path: GH_PAGES_DEPLOYMENT_TARGET_FILENAME,
          mode: '100644',
          content: GH_PAGES_DEPLOYMENT_TARGET_CONTENTS,
        },
      ],
    })
    const commit = await octokit.git.createCommit({
      owner,
      repo,
      parents: [],
      message: 'Publish website through Weird',
      tree: rootDir.data.sha,
    })
    await octokit.git.updateRef({
      owner,
      repo,
      ref,
      sha: commit.data.sha,
      force: true,
    })

    // Wait until we make sure the workflow runs. It might not be available immediately so we have a retry loop
    const after = Date.now()
    await new Promise(async (resolve) => {
      for (let i = 0; i < 40; i++) {
        try {
          await octokit.actions.createWorkflowDispatch({
            owner,
            repo,
            ref: 'refs/heads/' + defaultBranch,
            workflow_id: 'deploy.yaml',
          })
          resolve({})
          return
        } catch {}

        // Wait a short bit before retrying
        await new Promise((resolve) => {
          setTimeout(resolve, 250)
        })
      }
      throw "Site deployed, but couldn't trigger workflow"
    })

    goto(`/publish/github/status?repo=${repo}&after=${after}`)
  }

  const deploy = async () => {
    loading = true
    try {
      await deployImpl()
    } catch (e) {
      repoStatus = 'bad'
      errorMessage = e.toString()
    } finally {
      loading = false
    }
  }
</script>

{#if user}
  <div class="flex justify-center">
    <div class="flex items-center gap-4">
      <div class="avatar">
        <div class="w-10 rounded-full">
          <img alt="User Avatar" src={user.avatar_url} />
        </div>
      </div>
      <h2 class="text-lg">{user.name}</h2>
      <div class="tooltip tooltip-right" data-tip="Log Out">
        <button class="btn btn-ghost btn-square" on:click={logout}>
          <Icon name="logout" />
        </button>
      </div>
    </div>
  </div>

  <form class="flex flex-col items-stretch mt-8 max-w-xs mx-auto">
    <div class="w-full relative">
      <label class="label" for="repo">
        <span class="label-text">The repository to deploy to.</span>
      </label>
      <input
        id="repo"
        type="text"
        name="repo"
        placeholder="Repository Name"
        class="input input-bordered w-full"
        class:input-error={repoStatus == 'bad'}
        class:input-success={repoStatus == 'good_exists' ||
          repoStatus == 'good_will_create'}
        disabled={loading}
        bind:value={repo}
      />
      <div class="absolute top-[45px] right-[-42px]">
        {#if repoStatus == 'checking'}
          <Spinner />
        {/if}
      </div>
      <label class="label" for="repo">
        <span
          class="label-text-alt"
          class:text-error={repoStatus == 'bad'}
          class:text-success={repoStatus.includes('good')}
        >
          {statusMessage}&nbsp;
        </span>
      </label>
    </div>

    <div>
      <label class="label" for="domain">Domain</label>
      <input
        id="repo"
        type="text"
        name="domain"
        placeholder={`${user.login}.github.io/${repo}`}
        class="input input-bordered w-full"
        disabled={loading}
        bind:value={domain}
      />
    </div>

    <div class="flex items-center mt-6 fixed bottom-2 right-4">
      <div class="flex-grow" />
      {#if loading}
        <Spinner class="mr-2" />
      {/if}
      <button
        type="submit"
        class="btn btn-active btn-primary ml"
        disabled={loading ||
          !(repoStatus == 'good_exists' || repoStatus == 'good_will_create')}
        on:click={deploy}
      >
        Deploy
        <Icon name="rocket" class="ml-2" />
      </button>
    </div>
  </form>
{:else}
  <div class="flex justify-center mt-6">
    <Spinner size={40} />
  </div>
{/if}
