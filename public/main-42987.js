// TODO: clean this whole shit
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

// https://stackoverflow.com/questions/72237719/not-being-able-to-copy-url-to-clipboard-without-adding-the-protocol-https
function unsecuredCopyToClipboard(text) {
  const textArea = document.createElement("textarea");
  textArea.value = text;
  document.body.insertBefore(textArea, document.body.firstChild);
  textArea.focus();
  textArea.select();
  try {
    document.execCommand('copy');
  } catch (err) {
    console.error('Unable to copy to clipboard', err);
  }
  document.body.removeChild(textArea);
}

function onLoad() {
  const ipButton = document.getElementById('copy-ip')
  const ipText = document.getElementById('copy-ip-text')

  if (ipButton && ipText) {
    const ip = ipButton.getAttribute('data-ip')

    ipButton.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(ip)
      } catch {
        unsecuredCopyToClipboard(ip)
      }

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

  const { hostname, pathname } = window.location

  if (pathname !== '/') {
    document.getElementById('link-find')?.classList.add('text-slate-800')
  } else if (hostname.startsWith('ipv6.') || hostname.startsWith('6.')) {
    document.getElementById('link-ipv6')?.classList.add('text-slate-800')
  } else {
    document.getElementById('link-ipv4')?.classList.add('text-slate-800')
  }
}

document.addEventListener('DOMContentLoaded', onLoad)
