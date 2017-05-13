const express = require('express');
var app = express();

var drone_info = '';
var desktop_info = '';

app.get('/drone', function(req, res) {
  console.log('Requesting drone information');
  res.send(drone_info);
});

app.post('/drone', function(req, res) {
  console.log('Posting drone information');
  drone_info = req.ip;
  console.log(drone_info);
  res.send('');
});

app.get('/desktop', function(req, res) {
  console.log('Requesting desktop information');
  res.send(desktop_info);
});

app.post('/desktop', function(req, res) {
  console.log('Posting desktop information');
  desktop_info = req.ip;
  console.log(desktop_info);
  res.send('');
});

var port = parseInt(process.argv[2]);
app.listen(port);
