let scrollTopButton = document.getElementById("scrollUpButton");
window.onscroll = function() {
    if (document.body.scrollTop > 20 || document.documentElement.scrollTop > 20) {
        scrollTopButton.style.display = "inline-flex";
    } else {
        scrollTopButton.style.display = "none";
    }
};