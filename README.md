# Speakify

온세상이 스핔이다

- 웹 버전: https://zeetee1235.github.io/speakify/
- 입력 포맷: `PNG`, `JPEG`, `WebP`
- 출력 포맷: `GIF`

![Example](네르_Cuayo.gif)


## 사용법

# 윈도우

웹 버전을 이용해주세요

# linux

```bash
./speakify.sh <quality> <input_image>
```

품질 프리셋:
- `low`: `64x64`, `50` frames (빠른 미리보기)
- `mid`: `128x128`, `100` frames (균형)
- `high`: `256x256`, `150` frames (고품질)

예시:

```bash
./speakify.sh low photo.jpg
./speakify.sh mid image.png
./speakify.sh high portrait.webp
```

출력 파일은 입력 파일과 같은 경로에 `(입력파일명)_Cuayo.gif`로 생성됩니다.


## 알고리즘 

분석 문서: `docs/report/report_speakify.pdf`

## 크레딧

이 프로젝트는 [obamify](https://github.com/Spu7Nix/obamify)에서 영감을 받았습니다.

