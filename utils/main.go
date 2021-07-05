package utils

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/gofiber/fiber/v2"
)

var PageData map[string]string

func init() {
	PageData = make(map[string]string)

	PageData["/"] = `
		<meta property="og:title" content="mxrr.dev" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://mxrr.dev/" />
		<meta property="og:image" content="https://mxrr.dev/static/logo.gif" />`

	PageData["/forkedytg"] = `
		<meta property="og:title" content="Forked YouTube Gaming" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://mxrr.dev/forkedytg" />
		<meta property="og:image" content="https://raw.githubusercontent.com/mxrr/BetterYTG/master/src/assets/icons/BetterYTG_red_128.png" />`

	PageData["/projects"] = `
		<meta property="og:title" content="mxrr.dev - projects" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://mxrr.dev/projects" />
		<meta property="og:image" content="https://mxrr.dev/static/logo.gif" />`

	PageData["/about"] = `
		<meta property="og:title" content="mxrr.dev - about" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://mxrr.dev/about" />
		<meta property="og:image" content="https://mxrr.dev/static/logo.gif" />`

	PageData["404"] = `
		<meta property="og:title" content="mxrr.dev - 404" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://mxrr.dev/" />
		<meta property="og:image" content="https://mxrr.dev/static/PepegaSit.png" />`

}

func SendHtmlWithMeta(c *fiber.Ctx, path string) error {
	var metatags string
	if val, ok := PageData[path]; ok {
		metatags = val
	} else {
		metatags = PageData["404"]
	}
	indexHtml, err := ioutil.ReadFile("dist/index.html")
	if err != nil {
		fmt.Println(err)
	}

	indexString := string(indexHtml)
	indexWithMeta := strings.Replace(indexString, "<!--META_TAGS_GO_HERE-->", metatags, 1)

	c.Set("Content-Type", "text/html")
	return c.SendString(indexWithMeta)
}