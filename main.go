package main

import (
	"log"
	"mxrr-dev/api"
	"mxrr-dev/utils"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/compress"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/utahta/go-cronowriter"
)

func main() {
	config := fiber.Config{
		ServerHeader: "mxrr.dev",
	}

	w := cronowriter.MustNew("logs/%d-%m-%Y.log")

	app := fiber.New(config)
	app.Use(logger.New(logger.Config{
		Output: w,
	}))
	app.Use(logger.New())
	app.Use(compress.New(compress.Config{
		Next: func(c *fiber.Ctx) bool {
			return utils.CheckCompress(c.Path())
		},
		Level: compress.LevelBestSpeed,
	}))

	apiGroup := app.Group("/api")
	api.Initialise(apiGroup)

	app.Get("/", func(c *fiber.Ctx) error {
		return utils.SendHtmlWithMeta(c, "/")
	})
	app.Static("/", "./dist")
	app.Get("/static/styles.css", func(c *fiber.Ctx) error {
		return c.SendFile("./static/styles.css")
	})

	app.Static("/files", "./files")

	app.Get("*", func(c *fiber.Ctx) error {
		path := c.OriginalURL()
		return utils.SendHtmlWithMeta(c, path)
	})

	log.Fatal(app.ListenTLS(":3030", "./certs/cert.pem", "./certs/key.pem"))
}
