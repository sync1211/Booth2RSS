# Visual Studio support

ARG LAUNCHING_FROM_VS
ARG FINAL_BASE_IMAGE=${LAUNCHING_FROM_VS:+aotdebug}

# Base
FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS base
USER $APP_UID
WORKDIR /app
EXPOSE 8080


# Build
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    clang zlib1g-dev
ARG BUILD_CONFIGURATION=Release
ADD Booth2RSS /src/Booth2RSS
ADD Booth2RSS.Web /src/Booth2RSS.Web
WORKDIR /src
RUN dotnet restore "./Booth2RSS.Web/Booth2RSS.Web.csproj"
WORKDIR "/src/Booth2RSS.Web"
RUN dotnet build "./Booth2RSS.Web.csproj" -c $BUILD_CONFIGURATION -o /app/build

# Publish
FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "./Booth2RSS.Web.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=true

# Set up debugging for Visual Studio
FROM base AS aotdebug
USER root
RUN apt-get update \
   && apt-get install -y --no-install-recommends \
   gdb
USER app

# Final
FROM ${FINAL_BASE_IMAGE:-mcr.microsoft.com/dotnet/runtime-deps:10.0} AS final
WORKDIR /app
EXPOSE 8080
COPY --from=publish /app/publish .
ENTRYPOINT ["./Booth2RSS.Web"]
