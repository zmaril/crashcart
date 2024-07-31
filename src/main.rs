use rerun::{demo_util::grid, external::glam};
use openssh::{Session, KnownHosts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("crashcart_example").spawn()?;



    let points = grid(glam::Vec3::splat(-10.0), glam::Vec3::splat(10.0), 10);
    let colors = grid(glam::Vec3::ZERO, glam::Vec3::splat(255.0), 10)
        .map(|v| rerun::Color::from_rgb(v.x as u8, v.y as u8, v.z as u8));

    rec.log(
        "my_points",
        &rerun::Points3D::new(points)
            .with_colors(colors)
            .with_radii([0.5]),
    )?;

    let session = Session::connect_mux("zestdev", KnownHosts::Strict).await?;
    let ls = session.command("ls").output().await?;

    rec.log("ssh", &rerun::TextLog::new(String::from_utf8(ls.stdout).expect("server output was not valid UTF-8")))?;

    let whoami = session.command("whoami").output().await?;
    rec.log("ssh", &rerun::TextLog::new(String::from_utf8(whoami.stdout).expect("server output was not valid UTF-8")))?;

    session.close().await?;


    Ok(())
}