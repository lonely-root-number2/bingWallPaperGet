# bingWallPaperGet
rust写的一个获取bing壁纸并保存到MySQL的小工具。从 http://www.bingimg.cn/ 网站爬取。用到了正则表达式，MySQL等。

A small tool written by rust to get Bing wallpaper and save it to MySQL. From http://www.bingimg.cn/ Website crawling. Using regular expressions, MySQL and so on.

MySQL  Create table statement

CREATE TABLE `bingwpaper` (
  `today` varchar(255) DEFAULT NULL,
  `title` varchar(255) DEFAULT NULL,
  `detail` varchar(255) DEFAULT NULL,
  `src` varchar(255) DEFAULT NULL,
  `timestamp` int(20) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

today column express the time just like "2016-03-05",title column is just the title of photo,details column doesn't usually work,src column is the download address.
