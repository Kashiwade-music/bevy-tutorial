// Helper function to calculate tint and shade
function adjustColor(baseColor, targetColor, ratio) {
    const [baseR, baseG, baseB] = baseColor.match(/\w\w/g).map((c) => parseInt(c, 16));
    const [targetR, targetG, targetB] = targetColor.match(/\w\w/g).map((c) => parseInt(c, 16));
    const newR = baseR + Math.round((targetR - baseR) * ratio);
    const newG = baseG + Math.round((targetG - baseG) * ratio);
    const newB = baseB + Math.round((targetB - baseB) * ratio);
    return `rgb(${newR}, ${newG}, ${newB})`;
}

// Event listener to generate colors
document.getElementById('generateBtn').addEventListener('click', () => {
    const baseColor = document.getElementById('colorPicker_base').value.substring(1); // Remove '#'
    const targetColor = document.getElementById('colorPicker_target').value.substring(1); // Remove '#'
    const numGenerate = document.getElementById('numGenerate').value;
    const container = document.getElementById('colorContainer');
    container.innerHTML = ''; // Clear previous colors

    // Generate tint and shade colors
    for (let i = 0; i < numGenerate; i++) {
        const ratio = (1 - Math.pow(Math.pow(0.05, 1 / (numGenerate - 1)), i));
        const adjustedColor = adjustColor(baseColor, targetColor, ratio);
        const box = document.createElement('div');
        box.className = 'color-box';
        box.style.backgroundColor = adjustedColor;
        box.textContent = `${(ratio * 100).toFixed(2)}%`;
        container.appendChild(box);
    }
});
