{-# LANGUAGE OverloadedStrings #-}

module Main where
---------------------------------------------------------------------
import           Control.Applicative ((<$>))
import           Data.Monoid         ((<>), mconcat)
import           Prelude             hiding (id)
import qualified Text.Pandoc         as Pandoc
import           Text.Pandoc.Options
import qualified Data.Set as S

---------------------------------------------------------------------
import           Hakyll


---------------------------------------------------------------------
-- | Entry point
main :: IO ()
main = hakyll $ do
    match "images/*" $ do
        route   idRoute
        compile copyFileCompiler

    match "css/*" $ do
        route   idRoute
        compile compressCssCompiler

    -- Build tags
    tags <- buildTags "entries/*" (fromCapture "tags/*.html")


    match (fromList ["about.md"]) $ do
        route   $ setExtension "html"
        compile $ pandocCompiler
            >>= loadAndApplyTemplate "templates/default.html" defaultContext
            >>= relativizeUrls

    -- Render entries
    match "entries/*" $ do
        route   $ setExtension ".html"
        compile $ do
            pandocMathCompiler
                >>= loadAndApplyTemplate "templates/entry.html" (postCtx tags)
                >>= loadAndApplyTemplate "templates/default.html" defaultContext
                >>= relativizeUrls

    -- Entry list
    create ["entries.html"] $ do
        route idRoute
        compile $ do
            posts <- recentFirst =<< loadAll "entries/*"
            let ctx = constField "title" "Entries" <>
                        listField "posts" (postCtx tags) (return posts) <>
                        defaultContext
            makeItem ""
                >>= loadAndApplyTemplate "templates/entries.html" ctx
                >>= loadAndApplyTemplate "templates/default.html" ctx
                >>= relativizeUrls

    -- Entry tags
    tagsRules tags $ \tag pattern -> do
        let title = "Entries tagged " ++ tag

        route idRoute
        compile $ do
            posts <- recentFirst =<< loadAll pattern
            let ctx = constField "title" title <>
                        listField "posts" (postCtx tags) (return posts) <>
                        defaultContext
            makeItem ""
                >>= loadAndApplyTemplate "templates/entries.html" ctx
                >>= loadAndApplyTemplate "templates/default.html" ctx
                >>= relativizeUrls


    match "index.html" $ do
        route idRoute
        compile $ do
            posts <- fmap (take 10) . recentFirst =<< loadAll "entries/*"
            let indexCtx =
                    listField "posts" (postCtx tags) (return posts) <>
                    constField "title" "Home"                <>
                    defaultContext

            getResourceBody
                >>= applyAsTemplate indexCtx
                >>= loadAndApplyTemplate "templates/default.html" indexCtx
                >>= relativizeUrls

    match "templates/*" $ compile templateCompiler

    -- Render RSS feed
    create ["rss.xml"] $ do
        route idRoute
        compile $ do
            entries <- fmap (take 10) . recentFirst =<< loadAll "entries/*"
            renderRss (feedConfiguration "All posts") feedCtx entries


--------------------------------------------------------------------
postCtx :: Tags -> Context String
postCtx tags = mconcat
    [ dateField "date" "%B %e, %Y"
    , tagsField "tags" tags
    , defaultContext
    ]

feedCtx :: Context String
feedCtx = mconcat
    [ bodyField "description"
    , defaultContext
    ]


pandocMathCompiler :: Compiler (Item String)
pandocMathCompiler =
    let mathExtensions = [Ext_tex_math_dollars, Ext_tex_math_double_backslash,
                          Ext_latex_macros]
        defaultExtensions = writerExtensions defaultHakyllWriterOptions
        newExtensions = foldr S.insert defaultExtensions mathExtensions
        writerOptions = defaultHakyllWriterOptions {
                          writerExtensions = newExtensions,
                          writerHTMLMathMethod = MathJax ""
                        }
    in pandocCompilerWith defaultHakyllReaderOptions writerOptions



feedConfiguration :: String -> FeedConfiguration
feedConfiguration title = FeedConfiguration
    { feedTitle       = "Notes - " ++ title
    , feedDescription = "Notes on programming and math and maybe other stuff."
    , feedAuthorName  = "Nick Hamann"
    , feedAuthorEmail = "nick@wabbo.org"
    , feedRoot        = "http://www.wabbo.org"
    }
