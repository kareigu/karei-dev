package main

import (
	"apiv1"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/logger"
)

func main() {
	config := fiber.Config{
		ServerHeader: "mxrr.dev",
	}
	app := fiber.New(config)
	app.Use(logger.New())

	api := app.Group("/api")
	apiv1.InitialiseV1(api)

	app.Static("/", "./dist")
	app.Get("/static/styles.css", func(c *fiber.Ctx) error {
		return c.SendFile("./static/styles.css")
	})

	app.Static("/static", "./static")
	app.Static("/files", "./files")

	app.Get("*", func(c *fiber.Ctx) error {
		return c.SendFile("./dist/index.html")
	})

	log.Fatal(app.Listen("localhost:2000"))
}
