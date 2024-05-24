<script>
  import loadingIcon from "src/exeplorer_frontend/static/hourglass.gif";
  export let visible = false;
</script>

{#if visible}
  <div class="loading">
    <img src={loadingIcon} alt="Loading..." />
  </div>
{/if}

<style>
  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1000;
  }

  .loading img {
    width: 64px;
    height: 64px;
  }
</style>
