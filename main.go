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

	app.Get("*", func(c *fiber.Ctx) error {
		return c.SendString("404")
	})

	log.Fatal(app.Listen("localhost:2000"))
}
