# from the mkwebaxsum directory\
apt install nodejs npm -y
npm init -y
npm install -D tailwindcss postcss postcss-cli autoprefixer @tailwindcss/cli daisyui@latest flyonui@latest glob chokidar

####npx tailwindcss init     <- version 3


npx tailwindcss -i ./tailwind_input.css -o ./static/css/base_webapp.css --watch
npx tailwindcss -i ./tailwind_input.css -o ./static/css/base_webapp.css --minify


npx @tailwindcss/cli -i ./tailwind_input.css -o ./static/css/base_webapp.css \
--content "~/MediaKraken/docker/core/mkwebaxum/templates/**/*.{html,htm}" --content "~/MediaKraken/docker/core/mkwebaxum/src/**/*.rs" \
--minify --debug

npx @tailwindcss/cli -i ./tailwind_input.css -o ./static/css/base_webapp.css \
--content "~/MediaKraken/docker/core/mkwebaxum/templates/**/*.{html,htm}" --content "~/MediaKraken/docker/core/mkwebaxum/src/**/*.rs" --debug


npx tailwindcss -i ./tailwind_input.css -o ./static/css/base_webapp.css 

npx postcss

node tailwind_watch.mjs

npx tailwindcss \
  -i ./tailwind_input.css \
  -o ./static/css/base_webapp.css \
  --content "/root/MediaKraken/docker/core/mkwebaxum/templates/**/*.html" \
  --content "/root/MediaKraken/docker/core/mkwebaxum/src/**/*.rs" \
  --debug