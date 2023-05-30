<script lang="ts">
  import { browser } from '$app/environment'
  import { goto } from '$app/navigation'
  import { WebviewWindow } from '@tauri-apps/api/window'
  import { Body, fetch } from '@tauri-apps/api/http'
  import { GH_ACCESS_TOKEN_NAME, GH_CLIENT_ID } from 'config'
  import { onDestroy } from 'svelte'
  import { clipboard } from '@tauri-apps/api'
  const GH_WINDOW_NAME = 'github_authorize'

  let loginData: { url: string; userCode: string } | null = null
  let authPollTask = null
  onDestroy(async () => {
    const ghWindow = WebviewWindow.getByLabel(GH_WINDOW_NAME)
    try {
      // Ignore errors if the window isn't open
      ghWindow.close()
    } catch {}

    if (authPollTask) {
      console.info('Cancelling poll for auth token.')
      clearInterval(authPollTask)
    }
  })
  if (browser) {
    ;(async () => {
      if (localStorage.getItem(GH_ACCESS_TOKEN_NAME)) {
        goto('/publish/github')
      }

      const codeResp = await fetch<{
        device_code: string
        user_code: string
        verification_uri: string
        expires_in: string
        interval: string
      }>('https://github.com/login/device/code', {
        method: 'POST',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json',
        },
        body: Body.json({
          client_id: GH_CLIENT_ID,
          scope: 'public_repo,workflow',
        }),
      })

      if (!codeResp.ok) {
        throw `Error response from GitHub: ${codeResp.status}: ${codeResp.data}`
      }

      loginData = {
        url: codeResp.data.verification_uri,
        userCode: codeResp.data.user_code,
      }
      console.log('client_id', GH_CLIENT_ID, codeResp.data.device_code)

      const checkLoginToken = async () => {
        console.info('Polling for auth token')
        const resp = await fetch<
          | {
              access_token: string
              token_type: string
              scope: string
            }
          | { error: string; error_description: string; error_uri: string }
        >('https://github.com/login/oauth/access_token', {
          method: 'POST',
          headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
          },
          body: Body.json({
            client_id: GH_CLIENT_ID,
            device_code: codeResp.data.device_code,
            grant_type: 'urn:ietf:params:oauth:grant-type:device_code',
          }),
        })

        if (!resp.ok) {
          console.error('Error response from github:', resp.status, resp.data)
          clearInterval(authPollTask)
        }

        if ('error' in resp.data) {
          if (resp.data.error != 'authorization_pending') {
            throw `Error in GitHub response: ${resp.data.error}: ${resp.data.error_description} - ${resp.data.error_uri}`
          } else {
            console.debug('Authorization pending')
            return
          }
        }

        localStorage.setItem(GH_ACCESS_TOKEN_NAME, resp.data.access_token)

        goto('/publish/github')
      }

      console.info('Start polling for login token.')
      authPollTask = setInterval(
        checkLoginToken,
        (parseInt(codeResp.data.interval) + 1) * 1000
      )
    })()
  }

  let codeCopyText: 'Click to Copy' | 'Copied!' = 'Click to Copy'
  const copyCode = async () => {
    if (browser) {
      codeCopyText = 'Copied!'
      setTimeout(() => (codeCopyText = 'Click to Copy'), 500)
      clipboard.writeText(loginData!.userCode)
    }
  }

  const openGithubWindow = async () => {
    if (browser) {
      new WebviewWindow(GH_WINDOW_NAME, {
        center: true,
        title: 'GitHub: Authorize Weird',
        url: loginData.url,
      })
    }
  }
</script>

{#if loginData}
  <div class="flex flex-col items-center gap-12 justify-center mt-20">
    <div class="text-center">
      Copy this code and input it into the device activation page to authorize
      Weird.
    </div>

    <div>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        on:click={copyCode}
        class="text-4xl text-center bg-slate-600 p-4 rounded-xl cursor-pointer"
      >
        {loginData.userCode}
      </div>

      <div class="text-center text-gray-300 mt-1">
        {codeCopyText}
      </div>
    </div>

    <button
      on:click={openGithubWindow}
      class="btn btn-success !hover:bg-green-500"
    >
      Activate Device
    </button>
  </div>
{/if}
