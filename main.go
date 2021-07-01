package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

func main() {
	config := fiber.Config{
		ServerHeader: "mxrr.dev",
	}
	app := fiber.New(config)

	app.Static("/", "./dist")
	app.Get("/static/styles.css", func(c *fiber.Ctx) error {
		return c.SendFile("./static/styles.css")
	})

	app.Static("/static", "./static")

	app.Get("*", func(c *fiber.Ctx) error {
		return c.SendFile("./dist/index.html")
	})

	log.Fatal(app.Listen("localhost:2000"))
}
