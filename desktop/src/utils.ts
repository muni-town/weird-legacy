import { GH_ACCESS_TOKEN_NAME } from 'config'
import { Octokit } from '@octokit/rest'
import { browser, version } from '$app/environment'
import { goto } from '$app/navigation'

export function getOctokit(): Octokit {
  if (!browser) {
    throw "Octokit only works in client code";
  }

  const accessToken = localStorage?.getItem(GH_ACCESS_TOKEN_NAME) || null;

  // Make sure we have a GitHub access token
  if (!accessToken) {
    goto('/publish/github/login')
  }

  const octokit = new Octokit({
    auth: accessToken,
    userAgent: `Weird App ${version}`,
  })
  octokit.hook.wrap('request', async (request, options) => {
    // Set the GitHub API version.
    options.headers['X-GitHub-Api-Version'] = '2022-11-28'
    return request(options)
  })

  return octokit
}
