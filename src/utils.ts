export { generateCaptchaImage } from "./captcha-image-wasm"

export const getRandomCaptchaText = (length: number): string => {
  const chars = "ABCDEFGHJKLMNPQRSTUVWXYZabcdfghijkmnpqrstuvwxyz123456789" // Missing I, O, 0, l to avoid confusion
  let text = ""
  for (let i = 0; i < length; i++) {
    text += chars.charAt(Math.floor(Math.random() * chars.length))
  }
  return text
}
