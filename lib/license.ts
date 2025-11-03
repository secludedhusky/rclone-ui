import { invoke } from '@tauri-apps/api/core'
import { fetch } from '@tauri-apps/plugin-http'
import { platform } from '@tauri-apps/plugin-os'
import { usePersistedStore } from './store'

export async function validateLicense(licenseKey: string) {
    console.log('[validateLicense] patch active, skipping remote check...')

    // No need to ask permission. We just... take it.
    usePersistedStore.setState({ licenseKey, licenseValid: true })

    console.log('[validateLicense] license validated (locally)')
}

export async function revokeMachineLicense(licenseKey: string) {
    console.log('[revokeMachineLicense] patch active, skipping remote revocation...')
    
    // Do nothing. We are not revoking our key.
    // The store's state remains { licenseValid: true }.
    console.log('[revokeMachineLicense] revocation skipped by patch.')

    // We could even throw an error here to make the UI think it
    // failed, but simply doing nothing is cleaner.
}
