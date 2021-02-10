export async function getBaseUri(): Promise<string> {
  if (process.env.IS_LOCALHOST) return 'http://localhost:8000/compressed';
  return 'https://ffxiv-data.dlunch.net/compressed';
}
