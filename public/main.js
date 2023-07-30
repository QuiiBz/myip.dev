function onLoad() {
  const ipButton = document.getElementById('copy-ip')
  const ipText = document.getElementById('copy-ip-text')

  if (ipButton && ipText) {
    const ip = ipButton.getAttribute('data-ip')

    ipButton.addEventListener('click', async () => {
      await navigator.clipboard.writeText(ip)

      ipText.innerText = 'Copied!'

      setTimeout(() => {
        ipText.innerText = 'Click to copy'
      }, 1000)
    })
  }
}

document.addEventListener('DOMContentLoaded', onLoad)
