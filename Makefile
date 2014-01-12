include_dir = build
title = 'Implastic'

html:
	pandoc -s index.md -t html5 -o index.html \
		    --include-in-header $(include_dir)/header.html \
		    --include-before-body $(include_dir)/top.html \
		    --include-after-body $(include_dir)/footer.html \
		    --title-prefix $(title) \
			--smart

	pandoc -s music.md -t html5 -o music.html \
		    --include-in-header $(include_dir)/header.html \
		    --include-before-body $(include_dir)/top.html \
		    --include-after-body $(include_dir)/footer.html \
		    --title-prefix $(title) \
			--smart
