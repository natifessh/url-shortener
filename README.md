# url-shortener
url-shortener written in rust
#how to use
--cargo build
--cargo run
#to upload urls and get the shortened url
  curl -X POST http://127.0.0.1:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{"original": "https://www.youtube.com"}'
#to head to the real url 
  http://127.0.0.1:8080/....
  replace the dots with the response you got when you uploaded your url like 4fb5360e-d6e2-437d-b804-c06cec468496
  
  
    

