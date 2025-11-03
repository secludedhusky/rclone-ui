// 'invoke', 'fetch', and 'platform' imports removed. They are no longer needed.
import { usePersistedStore } from './store'

export async function validateLicense(licenseKey: string) {
    console.log('[validateLicense] patch active, skipping remote check...')

    // No need to ask permission. We just... take it.
    usePersistedStore.setState({ licenseKey, licenseValid: true })

    console.log('[validateLicense] license validated (locally)')
}

// We prefix 'licenseKey' with '_' to tell TypeScript we
// are intentionally not using this parameter.
export async function revokeMachineLicense(_licenseKey: string) {
    console.log('[revokeMachineLicense] patch active, skipping remote revocation...')
    
    // Do nothing. We are not revoking our key.
    console.log('[revokeMachineLicense] revocation skipped by patch.')
}
