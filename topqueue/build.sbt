lazy val root = (project in file(".")).
  settings(
    inThisBuild(List(
      organization := "app.penryu.blog",
      scalaVersion := "2.12.6",
      version      := "0.1.0-SNAPSHOT"
    )),
    name := "topqueue",
    libraryDependencies += "org.scalatest" %% "scalatest" % "3.0.5" % Test,
  )
