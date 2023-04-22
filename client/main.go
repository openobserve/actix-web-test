/*
{"level":"info","method":"GET","code":200,"took":2,"time":"2023-04-21T00:25:29+08:00","message":"/"}
{"level":"info","method":"GET","code":200,"took":0,"time":"2023-04-21T00:25:29+08:00","message":"/_license?human=false"}
{"level":"info","method":"GET","code":200,"took":0,"time":"2023-04-21T00:25:29+08:00","message":"/"}
{"level":"info","method":"HEAD","code":200,"took":0,"time":"2023-04-21T00:25:29+08:00","message":"/_index_template/nginx-log"}
{"level":"info","method":"PUT","code":200,"took":0,"time":"2023-04-21T00:25:30+08:00","message":"/_index_template/nginx-log"}
{"level":"info","method":"GET","code":200,"took":0,"time":"2023-04-21T00:25:30+08:00","message":"/_data_stream/nginx-log"}
{"level":"info","method":"GET","code":200,"took":0,"time":"2023-04-21T00:25:30+08:00","message":"/"}
{"level":"info","method":"POST","code":200,"took":85,"time":"2023-04-21T00:25:30+08:00","message":"/_bulk"}
{"level":"info","method":"POST","code":200,"took":12,"time":"2023-04-21T00:26:04+08:00","message":"/_bulk"}
{"level":"info","method":"POST","code":200,"took":7,"time":"2023-04-21T00:26:04+08:00","message":"/_bulk"}
*/

package main

import (
	"io"
	"io/ioutil"
	"net/http"
	"time"
)

func main() {
	var err error
	t := http.DefaultTransport.(*http.Transport).Clone()
	t.MaxIdleConns = 100
	t.MaxConnsPerHost = 100
	t.MaxIdleConnsPerHost = 100

	client := &http.Client{
		Timeout:   10 * time.Second,
		Transport: t,
	}

	res, err := client.Get("http://localhost:5080/api/default/")
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()
	res, err = client.Get("http://localhost:5080/api/default/_license?human=false")
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()

	res, err = client.Get("http://localhost:5080/api/default/")
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()

	res, err = client.Head("http://localhost:5080/api/default/_index_template/nginx-log")
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()

	req, err := http.NewRequest("PUT", "http://localhost:5080/api/default/_index_template/nginx-log", nil)
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()

	res, err = client.Do(req)
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()
	res, err = client.Get("http://localhost:5080/api/default/_data_stream/nginx-log")
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()
	res, err = client.Get("http://localhost:5080/api/default/")
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()
	res, err = client.Post("http://localhost:5080/api/default/_bulk", "application/json", nil)
	if err != nil {
		panic(err)
	}
	io.Copy(ioutil.Discard, res.Body)
	res.Body.Close()

}
