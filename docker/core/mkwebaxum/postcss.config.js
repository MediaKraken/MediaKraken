// postcss.config.js
module.exports = {
  plugins: [
    require('tailwindcss')({
      content: ['./templates/**/*.html', './src/**/*.rs'],
    }),
    require('autoprefixer'),
    require('postcss-reporter')({ clearReportedMessages: true }),
  ],
};
