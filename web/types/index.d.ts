interface DownloadResult {
  id: string,
  fileName: string,
  status: number,
  adjustedSize: string,
  byteUnit: string,
  host: string,
  url: string,
  progress: string
}

interface DownloadStatusElement {
  icon: string,
  name: string,
  color: string
}