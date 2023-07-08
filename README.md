# Multithreaded Web Server Exercise

This repository is my attemp at the multithreaded web server exercise in the rust book. While implementing I will make some changes to implementation in the hopes of making it better. (**Note:** The intent of this exercise is to practice multithreading and **not** to create a robust web server.)

### Changes
- The book suggested we compare whole strings from the request stream to check which route the browser is requesting. I think it will be better to separate the request string by the `space` character and check the first index to be `GET` and second index to be a specified route.
  - Instead of checking if the first line of the request stream is `"GET / HTTP/1.1"`. I want to separate this string by `" "` to get an iterator. Then I call `next()` on the iterator twice to get the `Method` and the `Route` the browser is requesting.