<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <div class="main-container">
        <div class="panel">
            <div class="content">
                Lorem ipsum dolor sit amet, consectetur adipisicing elit.
                Aliquam aperiam, atque, cumque ea ex fugit in ipsam modi
                necessitatibus quasi, quo tenetur! Accusantium ipsum nemo
                nostrum rerum temporibus in ipsam modi necessitatibus quasi, quo
                tenetur! Accusantium ipsum nemo nostrum rerum temporibus vitae
                voluptates. Lorem ipsum dolor sit amet, consectetur adipisicing
                elit. Beatae doloribus eaque id praesentium tenetur. Inventore
                modi, nisi? Aspernatur aut culpa distinctio ducimus fuga maxime
                nobis numquam quibusdam quis totam! Autem! Lorem ipsum dolor sit
                amet, consectetur adipisicing elit. Aliquam aperiam, atque,
                cumque ea ex fugit in ipsam modi necessitatibus quasi, quo
                tenetur! Accusantium ipsum nemo nostrum rerum temporibus vitae
                voluptates. Lorem ipsum dolor sit amet, consectetur adipisicing
                elit. Beatae doloribus eaque id praesentium tenetur. Lorem ipsum
                dolor sit amet, consectetur adipisicing elit. Aliquam aperiam,
                atque, cumque ea ex fugit in ipsam modi necessitatibus quasi,
                quo tenetur! Accusantium ipsum nemo nostrum rerum temporibus
                vitae voluptates. Lorem ipsum dolor sit amet, consectetur
                adipisicing elit. Beatae doloribus eaque id praesentium tenetur.
                Inventore modi, nisi? Aspernatur aut culpa distinctio ducimus
                fuga maxime nobis numquam quibusdam quis totam! Autem! Lorem
                ipsum dolor sit amet, consectetur adipisicing elit. Aliquam
                aperiam, atque, cumque ea ex fugit in ipsam modi necessitatibus
                quasi, quo tenetur! Accusantium ipsum nemo nostrum rerum
                temporibus vitae voluptates. Lorem ipsum dolor sit amet,
                consectetur adipisicing elit. Beatae doloribus eaque id
                praesentium tenetur. Inventore modi, nisi? Aspernatur aut culpa
                distinctio ducimus fuga maxime nobis numquam quibusdam quis
                totam! Autem! Lorem ipsum dolor sit amet, consectetur
                adipisicing elit. Aliquam aperiam, atque, cumque ea ex fugit in
                ipsam modi necessitatibus quasi, quo tenetur! Accusantium ipsum
                nemo nostrum rerum temporibus vitae voluptates. Lorem ipsum
                dolor sit amet, consectetur adipisicing elit. Beatae doloribus
                eaque id praesentium tenetur. Lorem ipsum dolor sit amet,
                consectetur adipisi elit. Beatae doloribus eaque id praesentium
                tenetur. Inventore modi, nisi? Aspernatur aut culpa distinctio
                ducimus fuga maxime nobis numquam quibusdam quis totam! Autem!
                Lorem ipsum dolor sit amet, consectetur adipisicing elit.
                Aliquam aperiam, atque, cumque ea ex fugit in ipsam modi
                necessitatibus quasi, quo tenetur! Accusantium ipsum nemo
                nostrum rerum temporibus vitae voluptates. Lorem ipsum dolor sit
                amet, consectetur adipisicing elit. Beatae doloribus eaque id
                praesentium tenetur. Lorem ipsum dolor sit amet, consectetur
                adipisi
            </div>
        </div>
        <div class="panel">Test</div>
        <div class="panel">
            <p>Test</p>
        </div>
    </div>
    <!--    <div class="relative flex h-full flex-col">-->
    <!--        <div-->
    <!--            v-if="dataStore.isFileOpen && !dataStore.draggedFilePath"-->
    <!--            class="flex h-full flex-col gap-2"-->
    <!--        >-->
    <!--            <div class="frame flex min-h-0 grow flex-col">-->
    <!--                <slot name="table"></slot>-->
    <!--            </div>-->
    <!--            <div class="flex items-start gap-2">-->
    <!--                <div class="flex flex-col gap-2">-->
    <!--                    <div class="frame">-->
    <!--                        <div class="h-5 w-5">-->
    <!--                            <IconInfo />-->
    <!--                        </div>-->
    <!--                    </div>-->
    <!--                    <div class="frame">-->
    <!--                        <div class="h-5 w-5"></div>-->
    <!--                    </div>-->
    <!--                </div>-->

    <!--                <div class="frame min-w-0 grow">-->
    <!--                    <slot name="info"></slot>-->
    <!--                </div>-->
    <!--            </div>-->
    <!--            <div class="flex items-center gap-2">-->
    <!--                <div class="frame">-->
    <!--                    <button-->
    <!--                        type="button"-->
    <!--                        @click="$router.push('/settings')"-->
    <!--                        class="m-0 flex h-5 w-5 items-center justify-center border-none bg-transparent p-0 hover:text-pink-700"-->
    <!--                    >-->
    <!--                        <IconSettings class="h-5 w-5" />-->
    <!--                    </button>-->
    <!--                </div>-->
    <!--                <div class="frame min-w-0 grow">-->
    <!--                    <slot name="status"></slot>-->
    <!--                </div>-->
    <!--            </div>-->
    <!--        </div>-->

    <!--        &lt;!&ndash; v-else Block &ndash;&gt;-->
    <!--        <div v-else class="flex h-full items-center justify-center">-->
    <!--            <div-->
    <!--                class="flex h-full w-full flex-col items-center justify-center gap-4 rounded-2xl border-2 border-dashed border-pink-700 bg-stone-50/75 dark:bg-stone-900/90"-->
    <!--            >-->
    <!--                <div-->
    <!--                    class="h-32 w-32 text-pink-700"-->
    <!--                    :class="{ 'animate-bounce': dataStore.draggedFilePath }"-->
    <!--                >-->
    <!--                    <IconParquetFile />-->
    <!--                </div>-->
    <!--                <p>{{ dataStore.draggedFilePath }}</p>-->
    <!--            </div>-->
    <!--        </div>-->
    <!--    </div>-->
</template>

<style scoped>
.main-container {
    display: grid;
    grid-template-rows: 1fr 256px 48px;
    grid-template-columns: 1fr;
    grid-gap: 0.5rem;
    align-items: center;
    justify-items: center;
    justify-content: stretch;
    padding: 0.5rem;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.panel {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    align-items: start;
    align-content: stretch;
    justify-content: start;
    background: rgba(242, 229, 237, 0.995);
    border: 1px solid #b6003c;
    width: 100%;
    height: 100%;
    padding: 0.5rem;
    min-height: 0;
    z-index: 0;
}

.content {
    height: 100%;
    overflow: auto;
}
</style>
