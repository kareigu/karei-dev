package apiv1

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/gofiber/fiber/v2"
)

type Projects struct {
	Projects []Project `json:"projects"`
}

type Project struct {
	Name  string `json:"name"`
	Img   string `json:"img"`
	Desc  string `json:"description"`
	Links Links  `json:"links"`
}

type Links struct {
	Git   string     `json:"git"`
	Other []MiscLink `json:"other"`
}

type MiscLink struct {
	Link string `json:"link"`
	Icon string `json:"icon"`
}

func projectsHandler(c *fiber.Ctx) error {
	jsonFile, err := os.Open("pagedata/projects.json")
	if err != nil {
		fmt.Println(err)
	}
	defer jsonFile.Close()

	byteValue, _ := ioutil.ReadAll(jsonFile)
	var projects Projects

	json.Unmarshal(byteValue, &projects)

	return c.JSON(projects)
}
