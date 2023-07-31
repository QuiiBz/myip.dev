// https://dev.to/jorik/country-code-to-flag-emoji-a21
function getFlagEmoji(countryCode) {
  if (countryCode === 'unknown') {
    return '🌎';
  }

  const codePoints = countryCode
    .toUpperCase()
    .split('')
    .map(char => 127397 + char.charCodeAt());

  return String.fromCodePoint(...codePoints);
}

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

  const findForm = document.getElementById('find-form')

  if (findForm) {
    findForm.addEventListener('submit', event => {
      event.preventDefault()

      const input = document.getElementById('find-input')

      if (input) {
        const value = input.value

        if (value) {
          window.location.href = `/${value}`
        }
      }
    })
  }

  const geoFlag = document.getElementById('geo-flag')

  if (geoFlag) {
    const countryCode = geoFlag.getAttribute('data-country')
    const emoji = getFlagEmoji(countryCode)

    geoFlag.innerText = emoji
  }
}

document.addEventListener('DOMContentLoaded', onLoad)
