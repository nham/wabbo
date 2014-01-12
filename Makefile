include_dir = build
output_dir = out
title = 'wabbo'

html:
	pandoc -s index.md -t html5 -o $(output_dir)/index.html \
		    --include-in-header $(include_dir)/header.html \
		    --include-before-body $(include_dir)/top.html \
		    --include-after-body $(include_dir)/footer.html \
		    --title-prefix $(title) \
			--smart

	pandoc -s music.md -t html5 -o $(output_dir)/music.html \
		    --include-in-header $(include_dir)/header.html \
		    --include-before-body $(include_dir)/top.html \
		    --include-after-body $(include_dir)/footer.html \
		    --title-prefix $(title) \
			--smart

	pandoc -s linalg.md -t html5 -o $(output_dir)/linalg.html \
		    --include-in-header $(include_dir)/header.html \
		    --include-before-body $(include_dir)/top.html \
		    --include-after-body $(include_dir)/footer.html \
		    --title-prefix $(title) \
			--smart \
			--mathjax
