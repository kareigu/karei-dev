package v1

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/gofiber/fiber/v2"
)

type Forks struct {
	Forks []Fork `json:"forks"`
}

type Fork struct {
	Version  string `json:"version"`
	Filename string `json:"filename"`
}

func forksHandler(c *fiber.Ctx) error {
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
