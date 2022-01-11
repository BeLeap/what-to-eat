use serenity::{self, client::{EventHandler, Context}, Client, model::{gateway::Ready, interactions::{Interaction, InteractionResponseType, application_command::ApplicationCommand}}};
use std::env;
use dotenv;
use rand::prelude::*;

static FOODS: &'static [&'static str] = &["피자", "햄버거", "치킨", "빵", "떡볶이", "순대", "설렁탕", "어묵", "어묵탕", "우동", "텐동", "설렁탕", "순대국", "선지국", "삼계탕", "돼지국밥", "서브웨이", "맥도날드", "롯데리아", "버거킹", "피자나라 치킨공주", "푸라닭", "BBQ", "BHC", "교촌치킨", "호식이 두마리 치킨", "이삭토스트", "토스트", "에그드랍", "갈비탕", "갈비탕", "칼국수", "족발", "보쌈", "족발", "보쌈", "짜장면", "탕수육", "짬뽕", "짬뽕", "볶음밥", "비빔밥", "오므라이스", "등갈비", "감자탕", "닭발", "닭꼬치", "백반정식", "코다리", "곱창", "막창", "대창", "곱도리탕", "초밥", "회", "경양식", "일식 돈까스", "가츠동", "규동", "히레까스", "로스까스", "라멘", "라면", "쌀국수", "마라탕", "우육탕면", "로제 파스타", "까르보나라", "까르보나라", "리조또", "스파게티", "편의점 도시락", "컵라면", "삼각김밥", "쭈꾸미", "김치찌개", "된장찌개", "닭볶음탕", "닭갈비", "닭갈비", "부대찌개", "파인애플 볶음밥", "파인애플 볶음밥", "김치 볶음밥", "간계밥", "간계밥", "오리고기", "스테이크", "아웃백", "삼겹살", "오겹살", "돼지껍데기", "목살", "아구찜", "매운탕", "찜닭", "마파두부", "물냉면", "비빔냉면", "만두", "판모밀", "카레", "돈부리", "곱창전골", "샤브샤브", "타코야끼", "오코노미야끼", "닭발", "쫄면", "제육볶음", "카레", "하이라이스", "조개구이", "장어덮밥", "사케동", "조개구이", "파인애플 피자", "하와이안 피자", "고구마 피자", "페퍼로니 피자", "도미노 피자", "피자 헛", "파파존스", "소고기죽", "전복죽", "호박죽", "잔치국수", "물냉면", "비빔냉면", "밀면", "온면", "간짜장", "간짜장", "사천짜장", "삼선짬뽕", "울면", "잡채밥", "계란 볶음밥", "마파두부", "묵밥", "육회", "불고기", "수육", "김피탕", "백숙", "장어구이", "고등어구이", "삼치구이", "물회", "소머리국밥", "떡국", "돼지국밥", "콩나물국밥", "소고기 무국", "곰탕", "닭개장", "도가니탕", "토마토 계란 볶음", "케밥", "군만두", "군만두", "군만두", "물만두", "닭가슴살 샐러드", "샌드위치", "굶는것", "다이어트"];

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "what" => {
                    FOODS[rand::thread_rng().gen_range(0..FOODS.len())].to_string()
                },
                _ => "not impl".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
            .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let _command = ApplicationCommand::create_global_application_command(&ctx.http, |command| {
            command.name("what").description("Let you know what to eat")
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
