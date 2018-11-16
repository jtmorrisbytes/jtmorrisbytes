this document outlines my decisions on accesing website resources on http://www.jtmorrisbytes.com

for urls, a <> denotes a placeholder, a <?> denotes an optional path and a <...> denotes a repeating pattern


I want to make this website as easily searchable as possible. as such, I want to reduce the usage of query strings whenever possible.

for static content, the structure could easily be
`http://www.jtmorrisbytes.com/<topic?>/<subtopic?...>/<static-page>`

an example of which would be
-   `http://www.jtmorrisbytes.com/home/` <- the homepage
-   `http://www.jtmorrisbyte.com/about`  <- possibly a portal page for the next two
-   `http://www.jtmorrisbytes.com/about/website` <- about the website
-   `http://www.jtmorrisbytes.com/about/author/`
-   `http://www.jtmorrisbytes.com/about/author/resume`


which may also work for dynamic content as such:
`http://www.jtmorrisbytes.com/projects/<project-name>/<sub-page?>/<parameters?...>`

-  `/` <- returns a page with a list of projects
-  `/sort/<sort-type>/<sort-direction>/<item-limit>/<page-number>`
    * `/sort/date/ascending/100/page-1` <- returns a page sorted by date in ascending order limiting the results to 100
-  `/the-angular-experiment/`

-  `http://www.jtmorrisbytes.com/projects/the-angular-experiment` <- returns a page outlining the project