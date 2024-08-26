import * as captchImage from "./rust/pkg/captcha_image"
import captchImageWasm from "./rust/pkg/captcha_image_bg.wasm"

captchImage.initSync(captchImageWasm)
export const generateCaptchaImage = captchImage.generate_image_from_text
