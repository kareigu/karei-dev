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
		<meta property="og:title" content="karei.dev" />
		<meta property="og:description" content="väsinud" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://karei.dev/" />
		<meta property="og:image" content="https://karei.dev/static/logo.gif" />`

	PageData["/forkedytg"] = `
		<meta property="og:title" content="Forked YouTube Gaming" />
		<meta property="og:description" content="ForkedYTG enhances YouTube livestreams with more emotes (Twitch, BTTV Emotes), new features, and more." />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://karei.dev/forkedytg" />
		<meta property="og:image" content="https://raw.githubusercontent.com/kareigu/BetterYTG/master/src/assets/icons/BetterYTG_red_128.png" />`

	PageData["/projects"] = `
		<meta property="og:title" content="karei.dev - projects" />
		<meta property="og:description" content="Various projects" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://karei.dev/projects" />
		<meta property="og:image" content="https://karei.dev/static/logo.gif" />`

	PageData["/about"] = `
		<meta property="og:title" content="karei.dev - about" />
		<meta property="og:description" content="About page" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://karei.dev/about" />
		<meta property="og:image" content="https://karei.dev/static/logo.gif" />`

	PageData["404"] = `
		<meta property="og:title" content="karei.dev - 404" />
		<meta property="og:description" content="Mis se vika on" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://karei.dev/" />
		<meta property="og:image" content="https://karei.dev/static/PepegaSit.png" />`

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

func CheckCompress(path string) bool {
	return strings.Contains(path, "/static") || strings.Contains(path, "/files")
}
