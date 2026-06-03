import axios from "axios";

var instance = axios.create({
  baseURL: "http://localhost:8080",
  timeout: 1000,
  headers: { Authorization: "foobar" },
});

export default instance;