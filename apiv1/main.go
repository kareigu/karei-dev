package apiv1

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/gofiber/fiber/v2"
)

func InitialiseV1(api fiber.Router) *fiber.Router {
	v1 := api.Group("/v1")

	v1.Get("/test", func(c *fiber.Ctx) error {
		return c.SendString("test")
	})
	v1.Get("/forks", forks)

	return &v1
}

type Forks struct {
	Forks []Fork `json:"forks"`
}

type Fork struct {
	Version  string `json:"version"`
	Filename string `json:"filename"`
}

func forks(c *fiber.Ctx) error {
	jsonFile, err := os.Open("pagedata/forks.json")
	if err != nil {
		fmt.Println(err)
	}
	defer jsonFile.Close()

	byteValue, _ := ioutil.ReadAll(jsonFile)
	var forks Forks

	json.Unmarshal(byteValue, &forks)

	return c.JSON(forks)
}
