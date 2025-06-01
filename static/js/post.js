fetch('http://localhost:8080/echo',{
    method: 'POST',
    headers: {
        'Content-Type': 'text/plain',
    },
    body: 'Hello webserver'
})
.then(response => response.text())
.then(data => console.log(data));

fetch('http://localhost:8080/echo', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    message: 'Hello',
    timestamp: new Date().toISOString()
  })
})
.then(response => response.text())
.then(data => console.log(data));
