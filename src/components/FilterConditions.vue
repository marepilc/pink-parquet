<script setup lang="ts">
import { Condition } from '~/types/app-types'

const props = defineProps<{
    modelValue: Condition
    dtype: string
}>()

const selectCondition = ref<Condition | null>(null)

const emit = defineEmits(['update:modelValue'])

function updateCondition(condition: Condition) {
    if (condition === selectCondition.value) {
        selectCondition.value = null
        emit('update:modelValue', null)
    } else {
        selectCondition.value = condition
        emit('update:modelValue', condition)
    }
}
</script>

<template>
    <div class="flex gap-1">
        <UButton
            :class="{
                'bg-pink-200 text-pink-700': selectCondition === Condition.lt,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.lt)"
        >
            <IconLt class="h-4 w-4" />
        </UButton>
        <UButton
            :class="{
                'bg-pink-200 text-pink-700': selectCondition === Condition.le,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.le)"
        >
            <IconLe class="h-4 w-4" />
        </UButton>
        <UButton
            :class="{
                'bg-pink-200 text-pink-700': selectCondition === Condition.eq,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.eq)"
        >
            <IconEq class="h-4 w-4" />
        </UButton>
        <UButton
            :class="{
                'bg-pink-200 text-pink-700': selectCondition === Condition.neq,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.neq)"
        >
            <IconNeq class="h-4 w-4" />
        </UButton>
        <UButton
            :class="{
                'bg-pink-200 text-pink-700': selectCondition === Condition.ge,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.ge)"
        >
            <IconGe class="h-4 w-4" />
        </UButton>
        <UButton
            :class="{
                'bg-pink-200 text-pink-700': selectCondition === Condition.gt,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.gt)"
        >
            <IconGt class="h-4 w-4" />
        </UButton>
        <UButton
            :class="{
                'bg-pink-200 text-pink-700':
                    selectCondition === Condition.between,
            }"
            size="xs"
            color="gray"
            square
            variant="outline"
            @click="updateCondition(Condition.between)"
        >
            <IconBetween class="h-4 w-4" />
        </UButton>
    </div>
</template>
