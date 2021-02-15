use ureq::Error;
use regex::Regex;
use time::Time;
use time::Date;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use mysql::*;
use mysql::prelude::*;

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    details:String,
    src:String,
    day:String,
    title:String,
}
fn main() {
    // 遇事不决，先写注释
    //1. 生成151页的地址并保存到vec
    //2. 根据页地址取到所有details和src保存到vec
    //3. 根据details生成excel day,title,src
    //4. 根据src下载图片 
    let start = now();
    // let mut f = OpenOptions::new().create(true).append(true).open("data.txt").unwrap();
    let page_nums = get_page_num();
    let mut page_url = Vec::new();
    page_url.reserve(page_nums as usize);
    println!("目前有{}页",page_nums);
    for i in 1..page_nums+1{
        println!("正在生成第{}页地址",i);
        page_url.push("http://www.bingimg.cn/list".to_string()+&i.to_string());
    }
    println!("页面地址生成完成");
    // println!("{}  {}",page_url[0],page_url[150]);
    let mut nodes:Vec<Node> = Vec::new();
    nodes.reserve((page_nums * 12) as usize);
    for i in 0 ..page_nums{
        println!("正在获取第{}个页面的details,页面地址为{}",i+1,page_url[i as usize]);
        match ureq::get(&page_url[i as usize]).call()
        { //"http://www.bingimg.cn/list1"
            Ok(resp)=>{
                match Regex::new(r#"<div class="thumbnail">\n          <a href="([\s\S]*?)"([\s\S]*?)" src="http://www.bingimg.cn/static/downimg/scale/SCALE.([\s\S]*?)" data-holde"#){
                    Ok(re)=>{
                        // println!("not err");
                        match resp.into_string(){
                            Ok(rstr)=>{
                                for caps in re.captures_iter(&rstr) {
                                    // println!("details:http://www.bingimg.cn{}   src:https://cn.bing.com/th?id={}",caps.get(1).unwrap().as_str(),caps.get(3).unwrap().as_str());
                                    let details = "http://www.bingimg.cn".to_string() + caps.get(1).unwrap().as_str();
                                    let src = "https://cn.bing.com/th?id=".to_string() + caps.get(3).unwrap().as_str();
                                    let mut r = get_day_and_title(&details);
                                    println!("获得:{}",r);
                                    let ret:Vec<&str> = r.split("，").collect();
                                    nodes.push(Node{
                                        details:details,
                                        src:src,
                                        day:ret[0].to_string(),
                                        title:ret[1].to_string(),
                                    });
                                    // let mut r = String::from(r#"2021-02-02，大格洛克纳山山峰前的土拨鼠，奥地利 (© SeppFriedhuber/Getty Images)"#);
                                    // let ret:Vec<&str> = r.split("，").collect();
                                    // println!("{:?},{:?}",ret[0],ret[1]);
                                }
                            },
                            Err(_)=>{
    
                            }
                        }
                        
                    },
                    Err(_)=>{
                        println!("err");
                    }
                }
            },
            _ =>{
    
            }
        }
    }
    // 文件写入方案
    // for i in nodes{
    //     // ins_txt(&mut f,i);
    // }
    // mysql方案
    ins_mysql(nodes);

    let end = now();
    println!("程序运行结束,共花费{}s",end - start);
    // println!("{}",unix_timestamp("2021-02-02"));
    // let nums = get_page_num();
    // println!("{}",nums);
    /*
   */
    /*
    
    */
    // write_img("https://cn.bing.com/th?id=OHR.ElPit_ZH-CN1174143508_1920x1080.jpg");
    
}
fn get_day_and_title(url:&str)->String{
    // "http://www.bingimg.cn/image/1612195200/206298/1"
    match ureq::get(url).call(){
        Ok(resp)=>{
            match resp.into_string(){
                Ok(s)=>{
                    match Regex::new(r#"a_img_copyright" title="([\s\S]*?)""#){
                        Ok(re)=>{
                            let mut ret = re.captures_iter(&s);
                            if let Some(x) =ret.next(){
                                println!("{}",x.get(1).unwrap().as_str());
                                return x.get(1).unwrap().as_str().to_string();
                            }       
                        },
                        Err(_)=>{

                        }
                    }
                },
                _ =>{

                }
            }
        },
        Err(_)=>{

        }
    }
    "".to_string()
}
fn ins_txt(f:& mut File,node:Node){ 
    f.write(format!("{}\t{}\t{}\t{}\t{}\n",node.day,node.title,node.details,node.src,unix_timestamp(&node.day)).as_bytes()).unwrap();
}

fn ins_mysql(mut payments_all:Vec<Node>){
    let url = "mysql://root:passwd.@localhost:3306/bingWallPaper";
    let pool = Pool::new(url).unwrap();
    let mut conn_p = pool.get_conn().unwrap();
    let conn = conn_p.as_mut();
    if conn.ping(){
        println!("连接成功");
    }else{
        println!("失败成功");
    }
    // 测试中超过当数量超过1000时一次将只能保存1000条，因此需要分2批次执行
    // 考，看错了，不需要
    // let payments:Vec<Node> = payments_all.drain(1000..).collect();
    conn.exec_batch(
        r"INSERT INTO bingwpaper (today, title, detail,src,timestamp)
          VALUES (:today, :title, :detail,:src,:timestamp)",
        payments_all.iter().map(|p| params! {
            "today" => &p.day,
            "title" => &p.title,
            "detail" => &p.details,
            "src" => &p.src,
            "timestamp" =>unix_timestamp(&p.day),
        })
    ).unwrap();
    // let payments = payments_all;
    // conn.exec_batch(
    //     r"INSERT INTO bingwpaper (today, title, detail,src,timestamp)
    //       VALUES (:today, :title, :detail,:src,:timestamp)",
    //     payments.iter().map(|p| params! {
    //         "today" => &p.day,
    //         "title" => &p.title,
    //         "detail" => &p.details,
    //         "src" => &p.src,
    //         "timestamp" =>unix_timestamp(&p.day),
    //     })
    // ).unwrap();
}
fn get_page_num()->u32{
   
    //23242
    match ureq::get("http://www.bingimg.cn/list1").call()
    {
        Ok(resp)=>{
           match resp.into_string(){
              Ok(s)=>{
                
                // println!("{},{}","--------------",&s.as_str()[23100..23242]);
                  match s.as_str().find("<li  ><a href=\"list2\">»</a></li>"){
                      Some(x)=>{
                            match s.as_str()[x-100..x].find("</a>"){
                                Some(y)=>{
                                    // println!("{},{}","--------------",&s.as_str()[23100..23242]);
                                    match s.as_str()[x-100..x].find("\">"){
                                        Some(z)=>{
                                                // println!("{},{}","----",&s.as_str()[x-100..x][z+2..y])
                                                return s.as_str()[x-100..x][z+2..y].to_string().parse::<u32>().unwrap();
                                        },
                                        None=>{

                                        }
                                    }
                                },
                                None=>{

                                }
                            }
                      },
                      None=>{
                          
                      }
                  }
              },
              _ =>{

              }
           }
         
        },
        Err(Error::Status(code, response)) => {
            /* the server returned an unexpected status
               code (such as 400, 500 etc) */
        }
        Err(_) => { /* some kind of io/transport error */ }
        
    }
        0
}

fn unix_timestamp(date_str:&str)->i64{
    match Date::parse(date_str,"%F"){
        Ok(t)=>{
            // println!("{}",t);
            // println!("{}",t.midnight().assume_utc().timestamp());
            return t.midnight().assume_utc().timestamp();
        },
        _=>{

        }
    }
    0
}

fn write_img(url:&str){
    // let url =  "https://cn.bing.com/th?id=OHR.ElPit_ZH-CN1174143508_1920x1080.jpg";
    let ret = ureq::get(url).call().unwrap();
    println!("the state code  =  {}",ret.status());
    let len = ret.header("Content-Length")
    .and_then(|s| s.parse::<usize>().ok()).unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(len);
    let n = ret.into_reader().read_to_end(&mut buf).unwrap(); 
    println!("the img size =  {}",n);
    let mut fs_pointer = File::create(r#"/root/img/a.jpg"#).unwrap();
    fs_pointer.write_all(&mut buf[0..n]);
    // let mut fs_pointer = File::open(r#"/root/test.txt"#).unwrap();
    // let mut contents = String::new();
    // fs_pointer.read_to_string(&mut contents).unwrap();
    // println!("{}",contents);
}   

fn now()->i64{
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs();
    ms as i64
}
