package api

import (
	v1 "mxrr-dev/api/v1"

	"github.com/gofiber/fiber/v2"
)

func Initialise(api fiber.Router) *fiber.Router {
	v1Group := api.Group("/v1")
	v1.Initialise(v1Group)

	return &v1Group
}
