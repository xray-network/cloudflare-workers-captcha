import { generateCaptchaImage, getRandomCaptchaText } from "@/utils"

export default {
  async fetch(request) {
    const requestURL = new URL(request.url)
    console.log(requestURL.pathname)

    if (requestURL.pathname === "/favicon.ico") {
      return new Response(null, { status: 404 })
    }

    if (requestURL.pathname !== "/") {
      const base64Image = generateCaptchaImage(requestURL.pathname.slice(1))
      return new Response(base64Image, {
        headers: {
          "content-type": "text/plain",
        },
      })
    }

    const base64Image = generateCaptchaImage(getRandomCaptchaText(6))
    return new Response(base64Image, {
      headers: {
        "content-type": "text/plain",
      },
    })
  },
} satisfies ExportedHandler
