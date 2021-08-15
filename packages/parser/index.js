const js = import("./pkg/parser")
js.then(js => {
  js.initialize().then(() => console.log(js.parse("text")))
})
