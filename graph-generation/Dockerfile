FROM maven:3.9.9 AS build
WORKDIR /build
COPY . .
RUN mvn clean package

FROM eclipse-temurin:latest
WORKDIR /generator
COPY --from=build /build/target/reggraphgen-1.0.0-jar-with-dependencies.jar /generator/reggraphgen.jar
EXPOSE 8080/tcp
CMD java $JVM_ARGS -jar /generator/reggraphgen.jar

