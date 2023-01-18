FROM node:latest as node
WORKDIR /app
COPY package.json package-lock.json .
RUN npm install
COPY . .
RUN npm run build --prod

FROM nginx:alpine
COPY --from=node /app/dist/studentifier-client /usr/share/nginx/html
COPY --from=node /app/src/nginx/nginx.conf /etc/nginx/conf.d/default.conf