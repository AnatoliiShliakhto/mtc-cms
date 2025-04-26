const scrollTopButton = document.getElementById("scrollUpButton");
window.addEventListener("scroll", () => {
    scrollTopButton.style.display = (document.body.scrollTop > 20 || document.documentElement.scrollTop > 20)
        ? "inline-flex"
        : "none";
});