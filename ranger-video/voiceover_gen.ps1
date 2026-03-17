Add-Type -AssemblyName System.Speech
$synth = New-Object System.Speech.Synthesis.SpeechSynthesizer

$outDir = "d:\bear strategy\ranger-accelerator\ranger-video\public"

# Slide 1: Thesis
$script1 = "Yield farming can leave you over-exposed to asset depreciation. Ranger Accelerator bundles a high-fidelity 25% plus APY yield stream with Delta-neutral hedging. Our edge? Utilizing Meteora dLMM fees and Drift perp shorts, we capture yield without exposure to SOL drawdown risk."
Write-Host "Generating voice1.wav..."
$synth.SetOutputToWaveFile("$outDir\voice1.wav")
$synth.Speak($script1)

# Slide 2: Operation
$script2 = "Here is how it works. First, the vault intakes USDC, 100% compliant with Seeding prize constraints. Second, looping: USDC is routed to Kamino Collateral to borrow JitoSOL securely. Third, Concentrated LP: we supply borrowed JitoSOL into Meteora dLMM harvesting absolute fee streams. Fourth, Short Hedge: locks position with Drift Perps Short protecting against the SOL long LP setup in parallel."
Write-Host "Generating voice2.wav..."
$synth.SetOutputToWaveFile("$outDir\voice2.wav")
$synth.Speak($script2)

# Slide 3: Risk Management
$script3 = "Risk integrity is paramount. We enforce Volume Capping via a custom max deposit cap. Time locking tracks rolling lockups accurately inside state buffers. And our Autonomous Rebalance Trigger node manages continuous liquidation prevention."
Write-Host "Generating voice3.wav..."
$synth.SetOutputToWaveFile("$outDir\voice3.wav")
$synth.Speak($script3)

# Slide 4: Outro
$script4 = "Ranger Accelerator is production viable. With verifiable build structures and scalable TVL coordination, we're ready for absolute pitch performance."
Write-Host "Generating voice4.wav..."
$synth.SetOutputToWaveFile("$outDir\voice4.wav")
$synth.Speak($script4)

$synth.Dispose()
Write-Host "All voiceovers generated successfully in $outDir"
