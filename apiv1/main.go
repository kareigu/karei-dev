package apiv1

import (
	"github.com/gofiber/fiber/v2"
)

func InitialiseV1(api fiber.Router) *fiber.Router {
	v1 := api.Group("/v1")

	v1.Get("/test", func(c *fiber.Ctx) error {
		return c.SendString("test")
	})
	v1.Get("/forks", forksHandler)
	v1.Get("/projects", projectsHandler)

	return &v1
}
