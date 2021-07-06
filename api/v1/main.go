package v1

import (
	"github.com/gofiber/fiber/v2"
)

func Initialise(v1 fiber.Router) *fiber.Router {
	v1.Get("/test", func(c *fiber.Ctx) error {
		return c.SendString("test")
	})
	v1.Get("/forks", forksHandler)
	v1.Get("/projects", projectsHandler)

	return &v1
}
