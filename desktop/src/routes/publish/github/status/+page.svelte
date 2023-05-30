<script lang="ts">
  import type { PageData } from './$types'
  import { browser } from '$app/environment'
  import Spinner from 'components/Spinner.svelte'
  import { getOctokit } from 'utils'
  import { type Octokit } from '@octokit/rest'
  import { WebviewWindow } from '@tauri-apps/api/window'

  let repo = ''
  let afterDate = ''
  let user:
    | Awaited<ReturnType<Octokit['users']['getAuthenticated']>>['data']
    | null = null
  let repoData: Awaited<ReturnType<Octokit['repos']['get']>>['data'] | null =
    null
  let domain = null
  let deployStatus = null
  let error: string | null = null

  let workflowId: number | null = null
  let jobsEtag: string | null = null
  let workflowEtag: string | null = null
  let workflowStatus: string | null = null
  let steps: { name: string; status: string }[] = []

  if (browser) {
    const url = new URL(window.location.href)
    repo = url.searchParams.get('repo')
    const after = parseInt(url.searchParams.get('after'))
    afterDate = after && new Date(after).toISOString()

    new Promise(async () => {
      const octokit = getOctokit()
      user = (await octokit.users.getAuthenticated()).data
      repoData = (
        await octokit.repos.get({
          owner: user.login,
          repo,
        })
      ).data
      domain =
        (
          await octokit.repos.getPages({
            owner: user.login,
            repo,
          })
        ).data.cname || `${user.login}.github.io/${repoData.name}`
      const owner = user.login

      // Start polling the workflow status
      while (true) {
        console.log('polling')
        try {
          if (workflowId && workflowEtag && workflowStatus) {
            try {
              const workflowRuns = await octokit.actions.listWorkflowRuns({
                owner,
                repo,
                workflow_id: 'deploy.yaml',
                created: afterDate ? `>${afterDate}` : undefined,
                exclude_pull_requests: true,
                per_page: 1,
                headers: {
                  'If-None-Match': workflowEtag,
                },
              })
              const workflowRun = workflowRuns.data.workflow_runs.at(0)
              if (workflowRun) {
                workflowId = workflowRun.id
                workflowStatus = workflowRun.status
              }
              workflowEtag = workflowRuns.headers.etag
            } catch (e: any) {
              if (e.status && e.status == 304) {
                throw 'wait'
              } else {
                console.error(e)
                throw 'Unexpected Error'
              }
            }
          } else {
            const workflowRuns = await octokit.actions.listWorkflowRuns({
              owner,
              repo,
              workflow_id: 'deploy.yaml',
              created: afterDate ? `>${afterDate}` : undefined,
              exclude_pull_requests: true,
              per_page: 1,
            })

            const workflowRun = workflowRuns.data.workflow_runs.at(0)
            if (workflowRun) {
              workflowId = workflowRun.id
              workflowStatus = workflowRun.status
            }
          }

          // The workflow isn't ready yet
          if (!workflowId) {
            throw 'wait'
          }

          let jobsList
          if (jobsEtag) {
            try {
              jobsList = await octokit.actions.listJobsForWorkflowRun({
                owner,
                repo,
                run_id: workflowId,
                headers: {
                  'If-None-Match': jobsEtag,
                },
              })
            } catch (e: any) {
              if (e.status && e.status == 304) {
                throw 'wait'
              } else {
                console.error(e)
                throw 'Unexpected Error'
              }
            }
          } else {
            jobsList = await octokit.actions.listJobsForWorkflowRun({
              owner,
              repo,
              run_id: workflowId,
            })
          }

          steps = []
          for (const job of jobsList.data.jobs) {
            for (const step of job.steps || []) {
              steps.push({
                name: step.name,
                status: step.status,
              })
            }
          }

          if (workflowStatus == 'completed') {
            console.log('done polling')
            break
          }
        } catch {}

        await new Promise((resolve) => setTimeout(resolve, 5000))
      }
    }).catch((e) => {
      error = e.toString()
    })
  }

  const openSite = async () => {
    if (browser) {
      console.log('show preview', domain)
      new WebviewWindow('gh-preview-window', {
        center: true,
        maximized: true,
        url: `https://${domain}`,
        title: `https://${domain}`,
      })
    }
  }
</script>

{#if error}
  <div class="text-error text-xl mt-4">
    Error: {error}
  </div>
{/if}

{#if !user || !repoData}
  <div class="flex justify-center mt-8">
    <Spinner size={40} />
  </div>
{:else}
  <div
    class="text-center font-medium my-4 flex justify-center gap-6 items-center"
  >
    Deployment Status
  </div>

  <div class="flex flex-col items-center gap-8">
    <div class="flex items-center gap-4">
      <div class="avatar">
        <div class="w-8 rounded-full">
          <img alt="User Avatar" src={user.avatar_url} />
        </div>
      </div>
      <h2 class="text-md">{user.name} / {repo}</h2>
    </div>
  </div>

  {#if workflowStatus == 'completed'}
    <div class="flex flex-col items-center prose text-center mx-auto pb-4">
      <div class="mt-12 mb-4 text-2xl font-bold">Deployment completed!</div>
      {#if domain != `${user.login}.github.io/${repo}`}
        <p>
          Your site has been deployed to GitHub Pages. If you have not done so
          already, you must make sure that your domain <code>{domain}</code> is
          configured with a <code>CNAME</code>
          record, pointing at <code>{user.login}.github.io/{repoData}</code>.
        </p>
        <p>
          Once that's done, you can access your new site at:
          <a class="link-primary" href={`https://${domain}`}>https://{domain}</a
          >!
        </p>
      {:else}
        <p>
          Your site has been deployed to GitHub pages. You can view it at <a
            href={`https://${domain}`}
            on:click={(e) => {
              e.preventDefault()
              openSite()
            }}
            on:keypress={(e) => {
              e.preventDefault()
              openSite()
            }}
            class="underline cursor-pointer">https://{domain}</a
          >.
        </p>
      {/if}
    </div>
  {:else}
    <h2
      class="text-xl text-center mt-4"
      class:text-success={workflowStatus == 'completed'}
      class:text-warning={workflowStatus != 'completed'}
    >
      {workflowStatus || ''}
    </h2>
    <div class="mt-4 flex justify-center">
      {#if steps.length > 0}
        <ul class="steps steps-vertical lg:steps-horizontal">
          {#each steps as step}
            <li
              class="step"
              class:step-primary={step.status == 'completed'}
              class:step-warning={step.status == 'in_progress'}
            >
              {step.name}
            </li>
          {/each}
        </ul>
      {:else}
        <div class="mx-auto">
          <Spinner size={50} />
        </div>
      {/if}
    </div>
  {/if}
{/if}
