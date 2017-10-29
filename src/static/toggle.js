$(".useradd-toggle").change(function () {
    const element = $(this);
    const uid = element.data('userid');

    console.log('toggling assignment status for: ', uid);

    const form = $("#hiddentoggle");
    $("#toggled-id").val(uid);
    form.submit();
})
