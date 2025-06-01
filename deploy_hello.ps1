# Deploy Hello Service and Client to Android Device
# 部署 Hello 服务和客户端到 Android 设备

Write-Host "🚀 Starting deployment of Hello Service and Client..." -ForegroundColor Green

# 检查设备连接
Write-Host "📱 Checking device connection..." -ForegroundColor Yellow
$devices = adb devices
if ($devices -match "device$") {
    Write-Host "✅ Android device connected" -ForegroundColor Green
} else {
    Write-Host "❌ No Android device found. Please connect your device and enable USB debugging." -ForegroundColor Red
    exit 1
}

# 定义文件路径
$TARGET_DIR = ".\target\aarch64-linux-android\release"
$DEVICE_DIR = "/data/local/tmp"
$HELLO_SERVICE = "$TARGET_DIR\hello_service"
$HELLO_CLIENT = "$TARGET_DIR\hello_client"

# 检查文件是否存在
Write-Host "🔍 Checking if binaries exist..." -ForegroundColor Yellow

if (-not (Test-Path $HELLO_SERVICE)) {
    Write-Host "❌ hello_service not found at $HELLO_SERVICE" -ForegroundColor Red
    Write-Host "💡 Please run: cargo ndk -t arm64-v8a build --release --bin hello_service" -ForegroundColor Cyan
    exit 1
}

if (-not (Test-Path $HELLO_CLIENT)) {
    Write-Host "❌ hello_client not found at $HELLO_CLIENT" -ForegroundColor Red
    Write-Host "💡 Please run: cargo ndk -t arm64-v8a build --release --bin hello_client" -ForegroundColor Cyan
    exit 1
}

Write-Host "✅ Both binaries found" -ForegroundColor Green

# 推送 hello_service
Write-Host "📤 Pushing hello_service to device..." -ForegroundColor Yellow
adb push $HELLO_SERVICE "$DEVICE_DIR/"
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ hello_service pushed successfully" -ForegroundColor Green
} else {
    Write-Host "❌ Failed to push hello_service" -ForegroundColor Red
    exit 1
}

# 推送 hello_client
Write-Host "📤 Pushing hello_client to device..." -ForegroundColor Yellow
adb push $HELLO_CLIENT "$DEVICE_DIR/"
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ hello_client pushed successfully" -ForegroundColor Green
} else {
    Write-Host "❌ Failed to push hello_client" -ForegroundColor Red
    exit 1
}

# 设置执行权限
Write-Host "🔧 Setting execute permissions..." -ForegroundColor Yellow
adb shell "chmod +x $DEVICE_DIR/hello_service"
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Execute permission set for hello_service" -ForegroundColor Green
} else {
    Write-Host "❌ Failed to set permission for hello_service" -ForegroundColor Red
}

adb shell "chmod +x $DEVICE_DIR/hello_client"
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Execute permission set for hello_client" -ForegroundColor Green
} else {
    Write-Host "❌ Failed to set permission for hello_client" -ForegroundColor Red
}

# 验证部署
Write-Host "🔍 Verifying deployment..." -ForegroundColor Yellow
$files = adb shell "ls -la $DEVICE_DIR/hello_*"
Write-Host "📋 Files on device:" -ForegroundColor Cyan
Write-Host $files

Write-Host ""
Write-Host "🎉 Deployment completed!" -ForegroundColor Green
Write-Host ""
Write-Host "📖 Usage instructions:" -ForegroundColor Cyan
Write-Host "  To run the service:" -ForegroundColor White
Write-Host "  To run the client:  adb shell '$DEVICE_DIR/hello_client'" -ForegroundColor White
Write-Host ""
Write-Host "💡 Tip: Run the service first, then run the client in another terminal" -ForegroundColor Yellow 