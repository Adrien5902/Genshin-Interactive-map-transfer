# How to use
1. Download the .exe converter from the [release tab](https://github.com/Adrien5902/Genshin-Interactive-map-transfer/releases/latest) and place it in a new empty folder
2. First open up the [official interactive map](https://act.hoyolab.com/ys/app/interactive-map/index.html)
3. Make sure every chest type you want to import is shown
4. Press `CTRL + SHIFT + I` to open up DevTools
5. Go the the `Networking` tab and make sure the recording dot is red
6. Reload the page (`CTRL + R` or `F5`)
7. Search for `list`
8. Find both:
  - `list?map_id=X&label_ids=X&app_sn=X&lang=X`
  - `mark_map_point_list?map_id=X&app_sn=X&lang=X`
   
    and right click them, then `Copy Response` 
9. Make two files in the folder named `list.json` and `mark_map_point_list.json` and paste in each one the copied content respectively 
10. Then go to the [unofficial interactive map](https://genshin-impact-map.appsample.com/) and repeat steps 3. to 6.
11. Search for `markers_all.v4.json` click and `Copy Response` again 
12. Paste the content into a new file called `markers_all.v4.json` in the same folder
13. Then run the .exe (if it fails feel free to create an [issue](https://github.com/Adrien5902/Genshin-Interactive-map-transfer/issues))
14. It will create a result.js file, copy the code inside and go back to the unofficial map
15. Open up the DevTools again and go to the `Console` tab
16. Paste in the js code and hit `Enter`
17. The site will process each chest one by one, it should take some time (~30min with most area explored at 100%) just leave it open and go touch some grass meanwhile
18. Once it's done no more `Marked location #XXX` notification should appear
> [!NOTE]
> Keep in mind that this will import only the main map and not the sub/underground areas such as the Chasm and Enkonomiya (if you need to import them do the process again with the map open in these areas)
 
